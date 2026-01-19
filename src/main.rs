use macroquad::prelude::*;

struct Territory {
    name: String,
    x_factor: f32,
    y_factor: f32,
    width_factor: f32,
    height_factor: f32,
    color: Color,
}

#[macroquad::main("Hazard2d")]
async fn main() {

    // Create the list of territories for Australia
    let territories = vec![
        // Australia Territories
        Territory {
            name: "Western Australia".to_string(),
            x_factor: 0.7,
            y_factor: 0.85,
            width_factor: 0.075,
            height_factor: 0.075,
            color: GREEN,
        },
        Territory {
            name: "Eastern Australia".to_string(),
            x_factor: 0.9,
            y_factor: 0.85,
            width_factor: 0.075,
            height_factor: 0.075,
            color: GREEN,
        },
        // Add more territories as needed
    ];

    loop {
        clear_background(BLUE);

        for territory in &territories {
            draw_rectangle(
                screen_width()*territory.x_factor,
                screen_height()*territory.y_factor,
                screen_width()*territory.width_factor,
                screen_height()*territory.height_factor,
                territory.color);
            draw_text(&territory.name,
                screen_width()*territory.x_factor,
                screen_height()*territory.y_factor,
                20.0,
                BLACK);
        }

        draw_text("Hazard", 20.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
}
