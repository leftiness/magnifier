use std::cmp;

use sdl2::video;

use magnifier::slide;

impl slide::Slide for video::Window {
    fn slide(&mut self, dx: i32, dy: i32) -> Result<(), slide::Error> {
        let index = self.display_index()
            .map_err(slide::Error::DisplayIndex)?;

        let bounds = self.subsystem()
            .display_bounds(index)
            .map_err(slide::Error::DisplayBounds)?;

        let (xpos, ypos) = self.position();
        let (xsize, ysize) = self.size();

        let xmin = bounds.left();
        let ymin = bounds.top();

        let xmax = bounds.right() - xsize as i32;
        let ymax = bounds.bottom() - ysize as i32;

        let x = cmp::min(cmp::max(xmin, xpos + dx), xmax);
        let y = cmp::min(cmp::max(ymin, ypos + dy), ymax);

        self.set_position(
            video::WindowPos::Positioned(x),
            video::WindowPos::Positioned(y),
        );

        Ok(())
    }
}
