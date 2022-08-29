use std::cmp;
use std::collections::{HashSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn bfs_can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(0);
        visited.insert(0);

        while queue.is_empty() == false {
            let cur = queue.pop_front().unwrap();
            for i in rooms[cur as usize].iter() {
                if visited.contains(i) == false {
                    queue.push_back(*i);
                    visited.insert(*i);
                    println!("cur={} visit={}", cur, *i);
                }
            }
        }

        rooms.len() == visited.len()
    }

    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        stack.push(0);
        visited.insert(0);

        while stack.is_empty() == false {
            let cur = stack.pop().unwrap();
            for i in rooms[cur as usize].iter() {
                if visited.contains(i) == false {
                    stack.push(*i);
                    visited.insert(*i);
                    println!("cur={} visit={}", cur, *i);
                }
            }
        }

        rooms.len() == visited.len()
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut count = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '1' && visited.contains(&(row, col)) == false {
                    count += 1;

                    println!("start={} {}", row, col);
                    queue.push_back((row, col));
                    visited.insert((row, col));

                    while queue.is_empty() == false {
                        let (r, c) = queue.pop_front().unwrap();
                        println!("pop={} {}", r, c);

                        if r >= 1 && grid[r - 1][c] == '1' && visited.contains(&(r - 1, c)) == false
                        {
                            // up
                            queue.push_back((r - 1, c));
                            visited.insert((r - 1, c));
                        }
                        if (r + 1) < grid.len()
                            && grid[r + 1][c] == '1'
                            && visited.contains(&(r + 1, c)) == false
                        {
                            // down
                            queue.push_back((r + 1, c));
                            visited.insert((r + 1, c));
                        }
                        if c >= 1 && grid[r][c - 1] == '1' && visited.contains(&(r, c - 1)) == false
                        {
                            // left
                            queue.push_back((r, c - 1));
                            visited.insert((r, c - 1));
                        }
                        if (c + 1) < grid[0].len()
                            && grid[r][c + 1] == '1'
                            && visited.contains(&(r, c + 1)) == false
                        {
                            // right
                            queue.push_back((r, c + 1));
                            visited.insert((r, c + 1));
                        }
                    }
                }
            }
        }
        count
    }

    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut stack = Vec::new();
        let orig_color = image[sr as usize][sc as usize];
        let mut image = image.clone();

        if new_color == orig_color {
            return image;
        }

        stack.push((sr as usize, sc as usize));
        image[sr as usize][sc as usize] = new_color;

        while stack.is_empty() == false {
            let (r, c): (usize, usize) = stack.pop().unwrap();
            println!("{} {}", r, c);

            if r >= 1 && image[r - 1][c] == orig_color {
                image[r - 1][c] = new_color;
                stack.push((r - 1, c));
            }
            if c >= 1 && image[r][c - 1] == orig_color {
                image[r][c - 1] = new_color;
                stack.push((r, c - 1));
            }
            if (r + 1) < image.len() && image[r + 1][c] == orig_color {
                image[r + 1][c] = new_color;
                stack.push((r + 1, c));
            }
            if (c + 1) < image[0].len() && image[r][c + 1] == orig_color {
                image[r][c + 1] = new_color;
                stack.push((r, c + 1));
            }
        }
        image
    }

    fn find_root(links: &Vec<usize>, node: i32) -> usize {
        let mut parent = links[node as usize];
        while parent != links[parent] {
            parent = links[parent];
        }
        parent
    }

    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut links: Vec<usize> = vec![0; n as usize];
        for i in 0..links.len() {
            links[i] = i;
        }

        // build link map
        for ed in edges {
            let r0 = Solution::find_root(&links, ed[0]);
            let r1 = Solution::find_root(&links, ed[1]);
            let parent = std::cmp::min(r0, r1);
            let child = std::cmp::max(r0, r1);
            links[child] = parent;
        }

        let mut root_set = HashSet::new();

        for ed in 0..n {
            let r = Solution::find_root(&links, ed);
            root_set.insert(r);
        }
        root_set.len() as i32
    }

    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut step = -1;
        let mut row = 0;
        let mut col = 0;

        if grid[0][0] == 0 {
            queue.push_back((0, 0, 1));
            visited.insert((0, 0));
        }

        while queue.is_empty() == false {
            let (r, c, s) = queue.pop_front().unwrap();
            row = r;
            col = c;
            if r == (grid.len() - 1) && c == (grid[0].len() - 1) {
                step = s;
            }
            println!("{} {} {}", r, c, s);

            for i in (r as i32 - 1)..=(r as i32 + 1) {
                for j in (c as i32 - 1)..=(c as i32 + 1) {
                    if i >= 0
                        && i < grid.len() as i32
                        && j >= 0
                        && j < grid[0].len() as i32
                        && grid[i as usize][j as usize] == 0
                        && visited.contains(&(i, j)) == false
                    {
                        visited.insert((i, j));
                        queue.push_back((i as usize, j as usize, s + 1));
                    }
                }
            }
        }
        step
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut edges = vec![vec![0; num_courses as usize]; num_courses as usize];
        for e in prerequisites {
            let s = e[0] as usize;
            let t = e[1] as usize;
            edges[s][t] = 1;
        }

        println!("{:?}", edges);

        let mut visited = HashSet::new();
        for i in 0..num_courses {
            if visited.contains(&i) == false {
                let mut cycle_path = HashSet::new();
                if Solution::dfs(i, &edges, &mut visited, &mut cycle_path) == true {
                    // detect cycle -> return false
                    return false;
                }
            }
        }
        // no cycle
        true
    }

    fn dfs(
        cur: i32,
        edges: &Vec<Vec<i32>>,
        visited: &mut HashSet<i32>,
        cycle_path: &mut HashSet<i32>,
    ) -> bool {
        println!("dfs={}", cur);
        if cycle_path.contains(&cur) == true {
            return true; // detect cycle
        }
        cycle_path.insert(cur);
        for i in 0..edges[cur as usize].len() as i32 {
            if edges[cur as usize][i as usize] == 1 {
                if visited.contains(&i) == true {
                    continue;
                }
                let ret = Solution::dfs(i, edges, visited, cycle_path);
                if ret == true {
                    println!("cycle={}", i);
                    return true; // detect cycle
                }
            }
        }
        println!("visit={} false", cur);
        visited.insert(cur);
        false
    }
}
