extern crate sdl2;
extern crate specs;
extern crate ncollide;

use sdl2::render::Renderer;

mod components;
mod systems;

use systems::system_render::system_render;
use components::component_render::ComponentRender;

//use specs::Join;

//#[derive(Clone, Debug)]
//struct Sum(usize);

pub fn init(width: u32, height: u32) ->  Renderer<'static> {
        let ctx = sdl2::init().unwrap();
		let video_ctx = ctx.video().unwrap();

		let window  = match video_ctx.window("Hook", width, height).position_centered().opengl().build() {
			Ok(window) => window,
			Err(_)   => panic!("failed to create window")
		};

		let renderer = match window.renderer().build() {
			Ok(renderer) => renderer,
			Err(_) => panic!("failed to create renderer")
		};

		renderer	
	}

fn main() {
    let mut planner = {
        let mut w = specs::World::new();
        // All components types should be registered before working with them
        w.register::<ComponentRender>();
        // create_now() of World provides with an EntityBuilder to add components to an Entity
        w.create_now().with(ComponentRender {x: 100, y: 100, w: 10, h: 10}).build();
        // build() returns an entity, we will use it later to perform a deletion
        

        // resources can be installed, these are nothing fancy, but allow you
        // to pass data to systems and follow the same sync strategy as the
        // component storage does.
        //w.add_resource(Screen(init(600,600)));

        // Planner is used to run systems on the specified world with a specified number of threads
        specs::Planner::<()>::new(w, 1)
    };

    {let screen = init(600,600);
    
        planner.run_custom(|arg| system_render(arg, screen));
    }
    

    // wait waits for all scheduled systems to finish
    // If wait is not called, all systems are run in parallel, waiting on locks
    //planner.wait();

    // Deletes an entity instantly
    // planner.mut_world().delete_now(e);

    // Instead of using macros you can use run_custom() to build a system precisely
    //planner.run_custom(|arg| {

        // Instead of using the `entities` array you can
        // use the `Join` trait that is an optimized way of
        // doing the `get/get_mut` across entities.
        //for (a, b) in (&mut sa, &sb).iter() {
        //    a.0 += if b.0 {2} else {0};
        //}

        // Dynamically creating and deleting entities
        //let e0 = arg.create();
        //sa.insert(e0, CompInt(-4));
        //let e1 = arg.create();
        //sa.insert(e1, CompInt(-5));
        //arg.delete(e0);
    //});

    
    //planner.run_custom(|arg| {
        //let (ints, mut count) = arg.fetch(|w| {
            //(w.read::<CompInt>(),
            // resources are acquired in the same way as components
            //w.write_resource::<Sum>())
        //});
        //count.0 = (&ints,).iter().count();
    //});
}