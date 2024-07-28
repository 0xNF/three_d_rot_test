use three_d::*;

pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Screen!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 1.3),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(vec![
            vec3(0.5, -0.5, 0.0),  // bottom right
            vec3(-0.5, -0.5, 0.0), // bottom left
            vec3(0.0, 0.5, 0.0),   // top
        ]),
        colors: Some(vec![
            Srgba::new(255, 0, 0, 255), // bottom right
            Srgba::new(0, 255, 0, 255), // bottom left
            Srgba::new(0, 0, 255, 255), // top
        ]),
        ..Default::default()
    };

    let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

    let mut gui = three_d::GUI::new(&context);
    let mut viewport_zoom = 1.0;
    let mut scissor_zoom = 1.0;
    let mut pitch: f32 = 0.0;
    let mut yaw: f32 = 0.0;
    let mut roll: f32 = 0.0;

    window.render_loop(move |mut frame_input| {
        let m_yaw = Mat4::from_angle_y(radians(yaw));
        let m_pitch = Mat4::from_angle_x(radians(pitch));
        let m_roll = Mat4::from_angle_z(radians(roll));

        let mf = Mat4::from_cols(m_yaw.y, m_yaw.y, m_roll.z, m_yaw.w);

        let mf = rotation_matrix_from_dir_to_dir(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(pitch, yaw, roll),
        );

        let mf = m_yaw * m_pitch * m_roll;

        // Mat4::
        // let mm = m4.y;
        // println!("x:{:#?}", m_yaw.z);
        // println!("y:{:#?}", mm.y);
        // println!("z:{:#?}", m4.z);
        // println!("w:{:#?}", m4.w);

        model.set_transformation(mf);

        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Debug Panel");
                    ui.add(Slider::new(&mut viewport_zoom, 0.01..=1.0).text("Viewport"));
                    ui.add(Slider::new(&mut scissor_zoom, 0.01..=1.0).text("Scissor"));
                    ui.add(Slider::new(&mut pitch, 0.0..=6.3).text("pitch"));
                    ui.add(Slider::new(&mut yaw, 0.0..=6.3).text("yaw"));
                    ui.add(Slider::new(&mut roll, 0.0..=6.3).text("roll"));
                });
                panel_width = gui_context.used_rect().width();
            },
        );

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };

        // Main view
        let viewport_zoomed = zoom(viewport_zoom, viewport);
        let scissor_box_zoomed = zoom(scissor_zoom, viewport).into();

        camera.set_viewport(viewport_zoomed);
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(1.0, 1.0, 1.0, 1.0, 1.0))
            .clear_partially(
                if viewport_zoom < scissor_zoom {
                    scissor_box_zoomed
                } else {
                    viewport_zoomed.into()
                },
                ClearState::color(0.8, 0.8, 0.8, 1.0),
            )
            .clear_partially(
                if viewport_zoom > scissor_zoom {
                    scissor_box_zoomed
                } else {
                    viewport_zoomed.into()
                },
                ClearState::color(0.5, 0.5, 0.5, 1.0),
            )
            .render_partially(scissor_box_zoomed, &camera, &model, &[])
            .write(|| gui.render())
            .unwrap();

        // Secondary view
        let secondary_viewport = Viewport {
            x: viewport.x,
            y: viewport.y,
            width: 200,
            height: 200,
        };
        camera.set_viewport(secondary_viewport);
        frame_input
            .screen()
            .clear_partially(
                secondary_viewport.into(),
                ClearState::color_and_depth(0.3, 0.3, 0.3, 1.0, 1.0),
            )
            .render_partially(secondary_viewport.into(), &camera, &model, &[]);

        // Returns default frame output to end the frame
        FrameOutput::default()
    });
}

fn zoom(zoom: f32, viewport: Viewport) -> Viewport {
    let width = (viewport.width as f32 * zoom) as u32;
    let height = (viewport.height as f32 * zoom) as u32;
    Viewport {
        x: ((viewport.width - width) / 2 + viewport.x as u32) as i32,
        y: ((viewport.height - height) / 2 + viewport.y as u32) as i32,
        width,
        height,
    }
}
