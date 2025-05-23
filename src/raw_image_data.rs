use bytemuck::Pod;

use crate::{
    pixel::{Pixel, SupportedChannels}, Channels, RawChannels
};

pub trait ImageData<'data> {
    fn n_pixels(&self) -> usize;
    fn iter_pixels<const N: usize>(&self) -> impl Iterator<Item = &'data [u8]>
    where
        Pixel<N>: SupportedChannels,
        [u8; N]: Pod;
    fn get_pixel_reader<const N: usize>(&self) -> fn(&mut Pixel<N>, &[u8]);
}

pub struct RawImageData<'data> {
    pub channels: RawChannels,
    pub data: &'data [u8],
    pub stride: usize,
    pub width: usize,
    pub height: usize,
}

impl<'data> ImageData<'data> for RawImageData<'data> {
    #[inline]
    fn iter_pixels<const N: usize>(&self) -> impl Iterator<Item = &'data [u8]> {
        let width = self.width;
        let bytes_per_pixel = self.bytes_per_pixel();
        self.data.chunks(self.stride).take(self.height).flat_map(move |row| {
            let pixel_row = &row[..width * bytes_per_pixel];
            pixel_row.chunks_exact(bytes_per_pixel)
        })
    }

    #[inline]
    fn n_pixels(&self) -> usize {
        self.height * self.width
    }

    #[inline]
    fn get_pixel_reader<const N: usize>(&self) -> fn(&mut Pixel<N>, &[u8]) {
        if N == 4 {
            if let Some(reader) = self.get_rgba_reader() {
                debug_assert_eq!(Channels::Rgba, self.channels.into());
                return reader;
            }
            debug_assert_eq!(Channels::Rgb, self.channels.into());
        }
        self.get_rgb_reader()
    }
}

impl RawImageData<'_> {
    #[inline]
    fn bytes_per_pixel(&self) -> usize {
        self.channels.bytes_per_pixel()
    }

    #[inline]
    fn get_rgba_reader<const N: usize>(&self) -> Option<fn(&mut Pixel<N>, &[u8])> {
        Some(match self.channels {
            RawChannels::Rgba => {
                |pixel, slice| pixel.update_rgba(slice[0], slice[1], slice[2], slice[3])
            }
            RawChannels::Argb => {
                |pixel, slice| pixel.update_rgba(slice[1], slice[2], slice[3], slice[0])
            }
            RawChannels::Bgra => {
                |pixel, slice| pixel.update_rgba(slice[2], slice[1], slice[0], slice[3])
            }
            RawChannels::Abgr => {
                |pixel, slice| pixel.update_rgba(slice[3], slice[2], slice[1], slice[0])
            }
            RawChannels::Rgbx
            | RawChannels::Xrgb
            | RawChannels::Rgb
            | RawChannels::Bgr
            | RawChannels::Bgrx
            | RawChannels::Xbgr => return None,
        })
    }

    #[inline]
    fn get_rgb_reader<const N: usize>(&self) -> fn(&mut Pixel<N>, &[u8]) {
        match self.channels {
            RawChannels::Rgb | RawChannels::Rgba | RawChannels::Rgbx => {
                |pixel, slice| pixel.update_rgb(slice[0], slice[1], slice[2])
            }
            RawChannels::Argb | RawChannels::Xrgb => {
                |pixel, slice| pixel.update_rgb(slice[1], slice[2], slice[3])
            }
            RawChannels::Bgr | RawChannels::Bgra | RawChannels::Bgrx => {
                |pixel, slice| pixel.update_rgb(slice[2], slice[1], slice[0])
            }
            RawChannels::Abgr | RawChannels::Xbgr => {
                |pixel, slice| pixel.update_rgb(slice[3], slice[2], slice[1])
            }
        }
    }
}
