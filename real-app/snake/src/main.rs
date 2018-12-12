extern crate piston;					//piston game engine library
extern crate graphics;				//piston 2d graphics library
extern crate glutin_window;				//Glut and GL are for graphics
extern crate opengl_graphics;
extern crate rand;						//used for random numbers

//Use allows use of modules listed below from the crates above
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone, PartialEq)] //allows comparison of directions
enum Direction
{
	Right, Left, Up ,Down
}

pub struct Game
{
	gl: GlGraphics,
	snake: Snake,
	coin : Coin,
	obj_width: u32,
	collide: bool,
}

impl Game
{
	//renders the background with a white screen
	fn render(&mut self, arg: &RenderArgs)
	{
		use graphics;

		let WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

		self.gl.draw(arg.viewport(), |_c, gl| {
			graphics::clear(WHITE, gl);
		});

		self.coin.render(&mut self.gl, arg, self.obj_width);
		self.snake.render(&mut self.gl, arg);
	}

	//updates the screen with snake and coin position
	//closes game if snake touches edge
	fn update(&mut self) -> bool
	{
		if !self.snake.update(self.collide)
		{
			return false;
		}

		if self.collide
		{
			self.collide = false;
		}

		self.collide = self.coin.collide(&self.snake);
		if self.collide
		{
			use rand::Rng;
            use rand::thread_rng;
            let mut r = thread_rng();
           	let rand_x = r.gen_range(1,10);
           	let rand_y = r.gen_range(1,10);
			self.coin = Coin{x_pos: rand_x, y_pos: rand_y};
		}
		return true;
	}

	//changes direction based on button presses
	fn pressed(&mut self, btn: &Button)
	{
		let last_direction = self.snake.dir.clone();

		self.snake.dir = match btn{
			&Button::Keyboard(Key::Up)
				if last_direction != Direction::Down => Direction::Up,
			&Button::Keyboard(Key::Down)
				if last_direction != Direction::Up => Direction::Down,
			&Button::Keyboard(Key::Left)
				if last_direction != Direction::Right => Direction::Left,
			&Button::Keyboard(Key::Right)
				if last_direction != Direction::Left => Direction::Right,
			_ => last_direction
		};
	}
}

pub struct Snake
{
	body: LinkedList<(u32, u32)>,
	width: u32,
	dir: Direction,
}

impl Snake
{
	//renders snake on screen
	fn render(&self, gl: &mut GlGraphics, args: &RenderArgs)
	{
		use graphics;

		let BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		let squares: Vec<graphics::types::Rectangle> = self.body
			.iter()
			.map(|&(x, y)|
			{
				graphics::rectangle::square(
					(((x - 1) * 50) + 5) as f64, 
					(((y - 1) * 50) + 5) as f64, 
					self.width.into())
				
			})
			.collect();


		gl.draw(args.viewport(), |c,gl| 
		{
			let transform = c.transform;
			squares.into_iter()
				.for_each(|square| graphics::rectangle(BLACK, square, transform, gl));
		});

	}
	//updates wheter snake has touched coin or edge
	pub fn update(&mut self, collide: bool)->bool
	{

		let mut new_head = (*self.body.front().expect("Snake has no body")).clone();	//clone or would be moving existing part

		match self.dir 
		{
			Direction::Left => new_head.0 -=1,
			Direction::Right => new_head.0 +=1,
			Direction::Up => new_head.1 -=1,
			Direction::Down => new_head.1 +=1,
		}

		self.body.push_front(new_head);		//moves snake head in front of old head, looks like it moves forward
		if !collide 
		{
			self.body.pop_back().unwrap();		//removes end of snake, keeps snake from growing from normal movement
		}

		if ((new_head.0 > 10) | (new_head.1 > 10))
		{
			return false;
		}
		else {
			return true
		}
	}
}

pub struct Coin
{
	x_pos: u32,
	y_pos: u32,
}

impl Coin 
{
	//detects collision with snake
	fn collide(&mut self, snake: &Snake) -> bool
	{
		let head = snake.body.front().unwrap();
		if head.0 == self.x_pos && head.1 == self.y_pos
		{
			true
		}else {
			false
		}
	}

	//renders coin on screen
	fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        use graphics;

        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        let x = ((self.x_pos-1)*50) + 5;
        let y = ((self.y_pos-1)*50) + 5;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(YELLOW, square, transform, gl)
        });
    }
}


fn main()
{
	const OBJ_WIDTH: u32 = 40;
	let opengl = OpenGL::V3_2;

	let mut window: GlutinWindow = WindowSettings::new("Snake", [500,500]).opengl(opengl).exit_on_esc(true).build().unwrap();
	
	let mut game = Game 
	{
		gl: GlGraphics::new(opengl),
		snake: Snake 
		{
			body: LinkedList::from_iter((vec![(5,5), (5,4)]).into_iter()),
			width: OBJ_WIDTH,
			dir: Direction::Right
		},
		coin: Coin
		{
			x_pos: 2, y_pos: 2,
		},
		obj_width: OBJ_WIDTH,
		collide: false,
	};

	//Event handling
	let mut events = Events::new(EventSettings::new()).ups(6);
	while let Some(e) = events.next(&mut window)
	{
		//Rendering events
		if let Some(r) = e.render_args()
		{
			game.render(&r);
		}
		//Game ending events
		if let Some(u) = e.update_args()
		{
			if !game.update()
			{
				break;
			}
		}
		//Button press events
		if let Some(k) = e.button_args()
		{
			if k.state == ButtonState::Press
			{
				game.pressed(&k.button);
			}
		}
	}
}

