pub mod camera;
pub mod image;
pub mod image_file;
pub mod point;

use crate::error::*;
use crate::scene;
pub use camera::*;
pub use image::*;
pub use image_file::*;
pub use point::*;
use std::fmt;
use std::io;

pub struct ColmapSource<R: io::Read + io::Seek> {
    pub cameras: Cameras,
    pub images: Images,
    pub image_files: ImageFiles<R>,
    pub points: Points,
}

impl<R: io::Read + io::Seek + Send + Sync> TryFrom<ColmapSource<R>>
    for scene::Scene
{
    type Error = Error;

    fn try_from(source: ColmapSource<R>) -> Result<Self, Self::Error> {
        use rayon::iter::{ParallelBridge, ParallelIterator};

        let points = source
            .points
            .into_iter()
            .map(|point| scene::Point {
                color: point.color_normalized(),
                position: point.position.to_owned(),
            })
            .collect();

        let views = source
            .images
            .values()
            .par_bridge()
            .map(|image| {
                let camera = {
                    let key = image.camera_id();
                    let value = source.cameras.get(key);
                    if value.is_none() {
                        return Err(Error::NoSuchCameraId(key.to_owned()));
                    }
                    value.unwrap()
                };
                let image_file_name = image.file_name().to_owned();
                let mut image_file = {
                    let value = source.image_files.get_mut(&image_file_name);
                    if value.is_none() {
                        return Err(Error::NoSuchImageFileName(
                            image_file_name,
                        ));
                    }
                    value.unwrap()
                };

                let projection_transform = match camera {
                    Camera::Pinhole(camera) => camera.projection_transform(),
                    _ => return Err(Error::Unimplemented),
                };
                let view_id = image.image_id().to_owned();
                let view_transform = image.view_transform();
                let image = image_file.read()?;

                let view = scene::View {
                    image,
                    image_file_name,
                    projection_transform,
                    view_id,
                    view_transform,
                };
                Ok((view_id, view))
            })
            .collect::<Result<_, _>>()?;

        Ok(scene::Scene { points, views })
    }
}

impl<R: io::Read + io::Seek> fmt::Debug for ColmapSource<R> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("ColmapSource")
            .field("cameras.len()", &self.cameras.len())
            .field("images.len()", &self.images.len())
            .field("image_files.len()", &self.image_files.len())
            .field("points.len()", &self.points.len())
            .finish()
    }
}
