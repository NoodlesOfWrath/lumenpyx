use lumenpyx::{lights::LightDrawable, *};

fn main() {
    let (mut lumen_program, event_loop) = LumenpyxProgram::new([128, 128]);

    let paths = vec![
        "../images/bricks_pixelated.png",
        "../images/test_sphere_heightmap.png",
        "../images/Border_Heightmap_Test.png",
    ];

    let mut drawables: Vec<Box<dyn Drawable>> = vec![];

    let mut lights = vec![Box::new(lights::DirectionalLight::new(
        [0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        2.0,
        0.001,
        0.01,
    ))];

    for path in paths {
        //let drawable = DrawableObject::new(path, path, path, &display, &indices, Transform::new());
        let drawable = Sprite::new(
            path.into(),
            path.into(),
            [0.0, 0.0, 0.0, 0.0].into(),
            &lumen_program.display,
            &lumen_program.indices,
            Transform::new([0.0, 0.0, 0.0]),
        );
        drawables.push(Box::new(drawable));
    }

    let mut t: f32 = 0.0;
    lumen_program.run(event_loop, |mut program| {
        {
            t += 0.01;
            lights[0].set_direction(t.cos(), t.sin(), 1.0);
        }

        let drawable_refs: Vec<&dyn Drawable> = drawables.iter().map(|d| d.as_ref()).collect();
        let light_refs: Vec<&dyn LightDrawable> =
            lights.iter().map(|l| &**l as &dyn LightDrawable).collect();

        draw_all(
            light_refs,
            drawable_refs,
            &mut program,
            &Camera::new([0.0, 0.0, 0.0]),
        );
    });
}
