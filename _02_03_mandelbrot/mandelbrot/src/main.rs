use num::Complex;
use std::str::FromStr;

/// The following function does this: Try to determine is 'c' is in the Mandelbrot set, using at most 'limit'
/// iterations to decide.
/// If 'C' is not a member, return some(i) where 'i' is the number of iterations it took for 'c' to leave the circle of radius 2 centered
/// on the origin. If 'c' seems to be a member (more precisely, if we reached the iteration limit without being able to prove that 'c'
/// is not a member), return None.
/// Option is an enumerated type (enum), because its definition enumerates several variants that a value could be: it is either Some(v) where v is of type T
/// or None. enum Option<T> {None, Some(T),}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> //Option<usize>: Returns Some(iteration_count) if c escapes within iteration_count iterations.
//Returns None if c remains bounded for the full limit iterations.
///usize is a built-in integer type that represents a size or index in memory. It is an unsigned integer type whose size 
/// depends on the architecture of the machine on which the program is running: On a 64-bit architecture, usize is 64 bits (8 bytes).
/// On a 32-bit architecture, usize is 32 bits (4 bytes).
{
    let mut z = Complex{re: 0.0, im: 0.0};
    for i in 0..limit
    {
        if z.norm_sqr() > 4.0 //norm_sqr is a method that calculated magnitude of the complex number.
        {
            return Some(i);
        }
        z = z * z + c;
    }
    None //If z is in the Mand.-set, None is returned.
}

/// The following lines handle the several CL arguments controlling the resolution of the image and parsing these arguments.
/// Parse the string 's' as a coordinate pair.
/// You can read the clause <T: FromStr> aloud as "For any type T that implements FromStr trait".
/// This effectively lets us define an entire family of functions at once: parse_pair::<i32>, parse_pair::<f64>.
/// <T>: Generics The angle brackets <...> declare that the function is generic over a type T.
/// Generics allow the function to work with any type, as long as the type meets specific requirements (in this case, implementing the FromStr trait).
/// : FromStr This is a trait bound, specifying that T must implement the FromStr trait. The FromStr trait provides functionality for parsing strings into values of a specific type.
/// Option<(T, T)>: Return type: If parsing is successful, it returns Some((T, T)) (a tuple of two T values). Otherwise, it returns None.
fn parse_pair<T: FromStr> (s: &str, separator: char) -> Option<(T, T)>
{
    match s.find(separator)//s.find(separator):The .find() method searches the string s for the first occurrence of the separator character.
    //Returns an Option<usize>: Some(index): If the separator is found, index is the position of the separator in the string.
    //None: If the separator is not found. T::from_str tries to parse two string slices in the tuple.
    //The tuple (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) is matched against the following patterns:
    //(Ok(l), Ok(r)):Both parsing operations succeeded (Ok variant).
    //The parsed values (l and r) are extracted and returned as a tuple inside Some.
    //_ (Wildcard): Any other combination of results (e.g., one or both parsing attempts failed) matches this pattern. Returns None.
    {
        None => None, //If the result of s.find(separator) is None (separator not found), the match expression also returns None.
        Some(index) => { //s[..index]: This is the substring from the start of s to (but not including) index
            //s[index + 1..]: This is the substring from one character past index to the end of s
            match(T::from_str(&s[..index]), T::from_str(&s[index + 1..]))
            //T::from_str attempts to parse a string slice into a value of type T
            {
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

//Test for parse_pair
fn test_parse_pair()
{
    assert_eq!(parse_pair::<i32>("",        ','), None);
    assert_eq!(parse_pair::<i32>("10,",     ','), None);
    assert_eq!(parse_pair::<i32>(",10",     ','), None);
    assert_eq!(parse_pair::<i32>("10,20",   ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// The following function uses the parse_pair function to parse a pair of floating point coordinates
/// and return them as Complex<f64>. If parse_pair succeeds, destructure the tuple into re (real part) 
/// and im (imaginary part). Construct a Complex<f64> number with Complex { re, im }. Return it wrapped in Some.
fn parse_complex(s: &str) -> Option<Complex<f64>>
{
    match parse_pair(s, ',')
    {
        Some((re, im)) => Some(Complex{re, im}),
        None => None
    }
}
// Test for parse_complex
#[test]
fn test_parse_complex()
{
    assert_eq!(parse_complex("1.25, -0.0625"), Some(Complex{re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// The following functions maps pixels to complex numbers.
/// The Mandelbrot set's mathematical definition works in the continuous space of the complex plane.
/// Example: The point 𝑐 = −0.5 + 0.5𝑖 is a point in the complex plane, not a pixel. 
/// Mapping Is the Bridge To compute whether a pixel should be part of the Mandelbrot set visualization:
/// We first map it to its corresponding complex number using pixel_to_point.
/// Then, we test the complex number using the Mandelbrot iterative algorithm.
/// The Complex Plane: We define a rectangular region of the complex plane to visualize, such as:
/// Upper-left corner:  (-2.0 + 1.0i) Lower-right corner: (1.0 - 1.0i)
/// This region corresponds to the part of the Mandelbrot set we want to compute.
/// Corresponding Coloring: Once each pixel is mapped to a complex number, the Mandelbrot algorithm determines:
/// Whether the number belongs to the Mandelbrot set (color it black). How quickly it escapes the set (color it based on escape speed).
fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), 
upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64>
//bounds: (usize, usize): The width and height of the image in pixels (e.g., bounds = (800, 600) for an 800×600 image).
// pixel: (usize, usize): The pixel's 2D coordinates in the image (e.g., (400, 300))
{
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex
    {
        re: upper_left.re + pixel.0 as f64 * width  / bounds.0 as f64,
        //pixel.0: The horizontal pixel index
        //pixel.0 as f64 ensures the horizontal pixel index is treated as a floating-point number.
        //The calculation scales pixel.0 (from 0 to bounds.0) to the corresponding range in the real axis of the complex plane (upper_left.re to lower_right.re).
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        //why subtraction here?
    }
}

#[test]
fn test_

fn main() {
    println!("Hello, world!");
}


fn complex_square_add_loop(c:Complex<f64>)
{
    let mut z = Complex{re: 0.0, im : 0.0}; //makes a struct 
    loop //Creates an infinite loop. The body of the loop will execute indefinitely unless explicitly broken out of.
    {
        z = z * z + c;
    }
}