use std::ops::{Add, Div, Mul, Sub};
use nannou::prelude::*;

struct Model {
    max_dist: f32,
    min_dist: f32,
    branches: Vec<Branch>,
    leaves: Vec<Leaf>,
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,

}
#[derive(Copy, Clone, PartialEq)]
struct Branch {
    parent_dir: Option<usize>,
    position: Vec3,
    direction: Vec3,
    length: f32,
    count: f32,
}
#[derive(Copy, Clone)]
struct Leaf {
    position: Vec3,
    reached: bool
}

impl Branch {
    fn next(self, parent_dir: usize) -> Branch {
        let next_dir =  Vec3::from(self.direction.mul(Vec3::from([self.length, self.length, self.length])));
        let next_pos =  Vec3::from(self.position.add(next_dir));
        let next_branch = Branch {
            parent_dir: Some(parent_dir),
            position: next_pos,
            direction: self.direction,
            length: 2.0,
            count: 0.0,
        };
        
        return next_branch
    }

    fn reset(mut self) -> Self{
        self.count = 0.0;
        return self
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}


fn model(_app: &App) -> Model{
    let mut leaves = vec![];
    leaves.push(Leaf{ position: Vec3::from([0.0, -320.0, 0.0]), reached: false });
    for _i in 0..=700 {

        let new_leaf = Leaf{ position: Vec3::from([random_range( -300.0, 300.0), random_range( -315.0, 450.0),random_range( -450.0, 450.0)]), reached: false };
      //  println!("{:?}", new_leaf.position);
        leaves.push(new_leaf);
    }

    let first_branch = Branch{ parent_dir: None, position: Vec3::from([-0.0,-380.0, 0.0]), direction: Vec3::from([0.0,0.0, 0.0]), length: 2.0, count: 0.0 };
    let second_branch = Branch{ parent_dir: None, position: Vec3::from([-50.0, -370.0, 50.0]), direction: Vec3::from([0.0, 0.0, 0.0]) , length: 5.0, count: 0.0 };
   // let third_branch = Branch{ parent_dir: None, position: [0.0,0.0], direction: [0.0,0.0], length: 5.0, count: 0.0 };
    Model{
        max_dist: 120.0,
        min_dist: 5.0,
        branches: vec![first_branch, second_branch],
        leaves,
        x_rotation: 0.0,
        y_rotation: 0.0,
        z_rotation: 0.0,
    }
}



fn update(_app: &App, model: &mut Model, _update: Update) {
    model.y_rotation += 0.0671;



    let mut new_leaves_vec = Vec::new();
    let mut new_branch_vec = Vec::new();

    for i in 0..model.leaves.len() {

        let mut closest_branch = None;
        let mut record = model.max_dist;
        let mut parent_directory = None;
        for j in  0..model.branches.len() {
            let distance1 = pow(model.branches[j].position[0] - model.leaves[i].position[0], 2) + pow(model.branches[j].position[1] - model.leaves[i].position[1], 2);
            let distance = distance1.sqrt() ;
         //   println!("{} {:?} {:?}", distance, model.branches[j].position, model.leaves[i].position);
            if distance < model.min_dist {
                model.leaves[i].reached = true;
                closest_branch = None;
                break

            } else if distance < record {

                closest_branch = Some(model.branches[j]);
                record = distance;
                parent_directory = Some(j);

            }

        }match closest_branch {
                Some(mut branch) => {

                    let new_dir =  Vec3::from(model.leaves[i].position.sub(branch.position)).normalize();
                    let final_dir =  Vec3::from(new_dir.add(branch.direction));
               //     println!("{}", new_dir);
                    branch.direction = final_dir;
                    branch.count +=1.0;
              //      println!("here {:?}", branch.direction);
                    match new_branch_vec.iter().position(|x| x == &branch.next(parent_directory.unwrap())){
                        Some(_number) => {},
                        None => {new_branch_vec.push(branch.next(parent_directory.unwrap()))}
                    }

                    },
                None => {},
            };
    }



    for i in &model.leaves {
        if i.reached == false {
            new_leaves_vec.push(*i);
        }
    }
    model.leaves = new_leaves_vec;
    for i in 0..model.branches.len() {
        let mut branch = model.branches[i];
        if branch.count > 0.0 {
            branch.direction = Vec3::from(branch.direction.div(Vec3::from([branch.count + 1.0, branch.count + 1.0, branch.count + 1.0])));
       //     println!("{:?}", branch.direction);
            match new_branch_vec.iter().position(|x| x == &branch.next(i)) {
                Some(_number) => {},
                None => {new_branch_vec.push(branch.next(i));
                    }

            } branch.reset();
        }
    }
    for i in new_branch_vec {
        match model.branches.iter().position(|x| x == &i) {
                Some(_number) => {},
                None => {model.branches.push(i);
                    }
        }

    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().line_mode();
    frame.clear(BLACK);
    for i in &model.branches {
        match i.parent_dir {
            Some(index) => {draw.mesh().points([i.position, model.branches[index].position]).turns(Vec3::from([model.x_rotation, model.y_rotation, model.z_rotation]));},
            None =>{}
        };

    }

    draw.to_frame(app, &frame).unwrap();
}