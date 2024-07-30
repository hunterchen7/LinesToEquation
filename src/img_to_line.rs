use image::{DynamicImage, GenericImage, GenericImageView};
use rand::Rng;

static WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
static BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);

pub fn get_image(path: &str) -> DynamicImage {
    image::open(path).unwrap()
} // fn get_image()

fn first_col(img: &DynamicImage, col: image::Rgba<u8>) -> Option<(u32, u32)> {
    let dims = img.dimensions();

    for x in 0..dims.0 {
        for y in 0..dims.1 {
            if img.get_pixel(x, y) == col {
                return Some((x, y));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn first_black(img: &DynamicImage) -> Option<(u32, u32)> {
    first_col(img, BLACK)
}

#[allow(dead_code)]
fn first_white(img: &DynamicImage) -> Option<(u32, u32)> {
    first_col(img, WHITE)
}

#[allow(dead_code)]
fn first_col_from(
    img: &DynamicImage,
    col: image::Rgba<u8>,
    start: (u32, u32),
) -> Option<(u32, u32)> {
    let dims = img.dimensions();

    for x in start.0..dims.0 {
        for y in start.1..dims.1 {
            if img.get_pixel(x, y) == col {
                return Some((x, y));
            }
        }
    }
    None
}

#[allow(dead_code)]
fn first_white_from(img: &DynamicImage, start: (u32, u32)) -> Option<(u32, u32)> {
    first_col_from(img, WHITE, start)
}

fn dfs(
    x: i32,
    y: i32,
    // visited: &mut Vec<Vec<bool>>,
    img: &mut Vec<Vec<bool>>,
    path: &mut Vec<(i32, i32)>,
) {
    if x < 0
        || y < 0
        || x >= img[0].len() as i32
        || y >= img.len() as i32
        // || visited[y as usize][x as usize]
        || !img[y as usize][x as usize]
    {
        // if out of bounds or already visited or not color match
        return;
    }
    // TODO: add path of (x, y) -> path[-1] to path, otherwise there are disconnected lines which means weird equations
    path.push((x, y));
    // path.push((x, y)); // add to path
    // visited[y as usize][x as usize] = true; // set visited
    img[y as usize][x as usize] = false; // set visited
    for (i, j) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        // loop through surrounding 3x3
        dfs(x + i, y + j, img, path);
    }
}

fn is_palindrome(s: usize, e: usize, path: &[(i32, i32)]) -> bool {
    let (mut start, mut end) = (s, e);
    while start < end {
        if path[start] != path[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

// some attempts at path shortening by removing end/beginning palindromic paths
fn remove_end_palindrome(path: &mut Vec<(i32, i32)>) {
    let longest_palindrome = (0..path.len()).rev().find(|&i| is_palindrome(0, i, path));
    if let Some(i) = longest_palindrome {
        path.truncate(path.len() - i / 2);
    }
}

fn remove_start_palindrome(path: &mut Vec<(i32, i32)>) {
    let longest_palindrome = (0..path.len()).rev().find(|&i| is_palindrome(0, i, path));
    if let Some(i) = longest_palindrome {
        // println!("i: {}", i);
        path.drain(0..i / 2);
    }
}

// takes in an image and returns a 2D vec of bools, true if pixel is color
// purpose is to reduce memory usage, since img contains 4 u8 per pixel
// compiler might end up optimizing this away anyway, but I think it's worth a shot
fn img_to_bool(img: &DynamicImage, col: image::Rgba<u8>) -> Vec<Vec<bool>> {
    let mut bool_img = vec![];
    for y in 0..img.height() {
        let mut row = vec![];
        for x in 0..img.width() {
            row.push(img.get_pixel(x, y) == col);
        }
        bool_img.push(row);
    }
    bool_img
}

// takes in an image and returns a 2D vec of points, each inner vec represents a line/path
pub fn edges_to_lines(img: &mut DynamicImage, col: image::Rgba<u8>) -> Vec<Vec<(i32, i32)>> {
    let mut lines = vec![];
    let dims = img.dimensions();
    // let mut visited = vec![vec![false; dims.0 as usize]; dims.1 as usize];
    let mut img = img_to_bool(img, col);

    for x in 0..dims.0 {
        for y in 0..dims.1 {
            if img[y as usize][x as usize] {
                let mut path = vec![];
                dfs(x as i32, y as i32, &mut img, &mut path);
                // the idea with these is that if dfs backtracks to a point that is already in the
                // path from the end/beginning of the path, then it can be partially truncated
                // remove_start_palindrome(&mut path);
                // remove_end_palindrome(&mut path);
                if path.len() > 16 {
                    lines.push(path);
                }
            }
        }
    }
    lines
}

#[allow(dead_code)]
pub fn edges_to_lines_w(img: &mut DynamicImage) -> Vec<Vec<(i32, i32)>> {
    edges_to_lines(img, WHITE)
}

#[allow(dead_code)]
pub fn edges_to_lines_b(img: &mut DynamicImage) -> Vec<Vec<(i32, i32)>> {
    edges_to_lines(img, BLACK)
}

// generates a random color, used in lines_to_img to visualize generated lines
fn random_col() -> image::Rgba<u8> {
    let col1 = rand::thread_rng().gen_range(100..255);
    let col2 = rand::thread_rng().gen_range(100..255);
    let col3 = rand::thread_rng().gen_range(100..255);
    image::Rgba([col1, col2, col3, 255])
}

// takes in a mutable image and a line, and colors the pixels in the line
// used in lines_to_img
fn line_to_img(img: &mut DynamicImage, line: &[(i32, i32)], col: image::Rgba<u8>) {
    for point in line.iter() {
        img.put_pixel(point.0 as u32, point.1 as u32, col);
    }
}

// takes in a 2D vec of points where the inner vectors represent lines
pub fn lines_to_img(lines: &[Vec<(i32, i32)>]) {
    let (mut max_x, mut max_y) = (0, 0); // find max x and y to set image dimensions
    for line in lines.iter() {
        for point in line.iter() {
            if point.0 > max_x {
                max_x = point.0;
            }
            if point.1 > max_y {
                max_y = point.1;
            }
        }
    }
    // + 50 for a bit of padding, might need a better system for this
    // TODO?
    let mut img = DynamicImage::new_rgb8(max_x as u32 + 50, max_y as u32 + 50);
    for line in lines.iter() {
        let col = random_col();
        line_to_img(&mut img, line, col);
    }
    img.save("generated/lines.png").unwrap();
} // fn lines_to_img()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_end_palindrome() {
        let mut path1 = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
        remove_end_palindrome(path1.as_mut());
        assert_eq!(
            path1,
            vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]
        );
    }

    #[test]
    fn test_remove_end_palindrome2() {
        let mut path2 = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (5, 5),
            (4, 4),
            (3, 3),
            (2, 2),
            (1, 1),
            (0, 0),
        ];
        remove_end_palindrome(path2.as_mut());
        assert_eq!(
            path2,
            vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]
        );
    }

    #[test]
    fn test_remove_end_palindrome3() {
        let mut path2 = vec![
            (0, 0),
            (1, 1),
            (0, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (5, 5),
            (4, 4),
            (3, 3),
            (2, 2),
            (0, 1),
            (1, 1),
            (0, 0),
        ];
        remove_end_palindrome(path2.as_mut());
        assert_eq!(
            path2,
            vec![
                (0, 0),
                (1, 1),
                (0, 1),
                (2, 2),
                (3, 3),
                (4, 4),
                (5, 5),
                (6, 6)
            ]
        );
    }

    #[test]
    fn test_remove_start_palindrome() {
        let mut path1 = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];
        remove_start_palindrome(path1.as_mut());
        assert_eq!(
            path1,
            vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)]
        );
    }

    #[test]
    fn test_remove_start_palindrome2() {
        let mut path2 = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (5, 5),
            (4, 4),
            (3, 3),
            (2, 2),
            (1, 1),
            (0, 0),
        ];
        remove_start_palindrome(path2.as_mut());
        assert_eq!(
            path2,
            vec![(6, 6), (5, 5), (4, 4), (3, 3), (2, 2), (1, 1), (0, 0)]
        );
    }

    #[test]
    fn test_remove_start_palindrome3() {
        let mut path2 = vec![
            (0, 0),
            (1, 1),
            (0, 0),
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (5, 5),
            (4, 4),
            (3, 3),
            (2, 2),
            (1, 1),
            (0, 0),
        ];
        remove_start_palindrome(path2.as_mut());
        assert_eq!(
            path2,
            vec![
                (1, 1),
                (0, 0),
                (2, 2),
                (3, 3),
                (4, 4),
                (5, 5),
                (6, 6),
                (5, 5),
                (4, 4),
                (3, 3),
                (2, 2),
                (1, 1),
                (0, 0)
            ]
        );
    }
}
