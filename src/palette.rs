pub struct Pal
{
    data: Vec<(u8, u8, u8)>,
}

impl Pal
{
    pub fn new(fname: Option<&str>) -> Result<Pal, String>
    {
        let data: Vec<(u8, u8, u8)> = if let Some(name) = fname
        {
            let img = image::open(name).map_err(|e| format!("Can not open palette image: {}", e))?.to_rgb();

            let width = img.width();
            let raw: Vec<u8> = img.into_raw();

            (0..width as usize).map(|i| (raw[i*3], raw[i*3 + 1], raw[i*3 + 2])).collect()
        }
        else
        {
            (0..256).map(|i| (i as u8, i as u8, i as u8)).collect()
        };

        Ok(Pal
        {
            data: data,
        })
    }

    pub fn get_col(&self, ind: f32, the_max: f32) -> (u8, u8, u8)
    {
        let end_ind: usize = (ind*self.data.len() as f32/the_max) as usize;
        self.data[if end_ind >= self.data.len() {self.data.len() - 1} else {end_ind}]
    }
}