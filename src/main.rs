use macroquad::prelude::*;
use std::collections::LinkedList;

const SQUARES: i16 = 16;

type Point = (i16, i16);

struct Snake {
    dir: Point,
    head: Point,
    body: LinkedList<Point>,
}

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = Snake {
        dir: (0, 1),
        head: (0, -1),
        body: LinkedList::new(),
    };

    let mut fruit = generate_fruit();
    let mut score = 0;
    let mut speed = 0.1;
    let mut last_update = get_time();
    let mut gameover = false;

    let _up = (0, -1);
    let _down = (0, 1);
    let _left = (-1, 0);
    let _right = (1, 0);

    loop {
        if !gameover {
            handle_input(&mut snake);

            if get_time() - last_update > speed {
                last_update = get_time();

                update_snake(
                    &mut snake,
                    &mut fruit,
                    &mut score,
                    &mut speed,
                    &mut gameover,
                );
            }
        }

        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));
        let grid_size = screen_width() / (SQUARES as f32);

        draw_grid(grid_size);
        draw_snake(&snake, grid_size);
        draw_fruit(fruit, grid_size);
        draw_score(score);

        if gameover {
            if is_key_pressed(KeyCode::R) {
                gameover = false;

                snake = Snake {
                    dir: (0, 1),
                    head: (0, -1),
                    body: LinkedList::new(),
                };

                fruit = generate_fruit();
                score = 0;
                speed = 0.1;
                last_update = get_time();
            }

            draw_gameover();
        }

        next_frame().await;
    }
}

fn generate_fruit() -> Point {
    (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES))
}

fn handle_input(snake: &mut Snake) {
    if is_key_down(KeyCode::W) && snake.dir != (0, 1) {
        snake.dir = (0, -1);
    } else if is_key_down(KeyCode::S) && snake.dir != (0, -1) {
        snake.dir = (0, 1);
    } else if is_key_down(KeyCode::A) && snake.dir != (1, 0) {
        snake.dir = (-1, 0);
    } else if is_key_down(KeyCode::D) && snake.dir != (-1, 0) {
        snake.dir = (1, 0);
    }
}

fn update_snake(
    snake: &mut Snake,
    fruit: &mut Point,
    score: &mut i16,
    speed: &mut f64,
    gameover: &mut bool,
) {
    snake.body.push_front(snake.head);
    snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);

    if snake.head == *fruit {
        *fruit = generate_fruit();
        *score += 1;
        *speed *= 0.95;
    } else {
        snake.body.pop_back();
    }

    if snake.head.0 < 0 || snake.head.1 < 0 || snake.head.0 >= SQUARES || snake.head.1 >= SQUARES {
        *gameover = true;
    }

    for (x, y) in &snake.body {
        if *x == snake.head.0 && *y == snake.head.1 {
            *gameover = true;
        }
    }
}

fn draw_grid(grid_size: f32) {
    let grid_color = Color::new(0.4, 0.4, 0.4, 1.0); // Цвет сетки

    for i in 1..SQUARES {
        let pos = i as f32 * grid_size;

        draw_line(pos, 0.0, pos, screen_height(), 1.0, grid_color);
        draw_line(0.0, pos, screen_width(), pos, 1.0, grid_color);
    }
}

fn draw_snake(snake: &Snake, grid_size: f32) {
    for (x, y) in &snake.body {
        draw_rectangle(
            (*x as f32) * grid_size,
            (*y as f32) * grid_size,
            grid_size,
            grid_size,
            Color::new(0.0, 0.8, 0.0, 1.0),
        );
    }

    draw_rectangle(
        snake.head.0 as f32 * grid_size,
        snake.head.1 as f32 * grid_size,
        grid_size,
        grid_size,
        Color::new(0.0, 1.0, 0.0, 1.0),
    );
}

fn draw_fruit(fruit: Point, grid_size: f32) {
    draw_rectangle(
        fruit.0 as f32 * grid_size,
        fruit.1 as f32 * grid_size,
        grid_size,
        grid_size,
        Color::new(1.0, 0.0, 0.0, 1.0),
    );
}

fn draw_score(score: i16) {
    let score_text = format!("Score: {}", score);
    draw_text(
        &score_text,
        30.0,
        30.0,
        32.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
}

fn draw_gameover() {
    draw_text(
        "Game Over",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0,
        48.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    );
}
