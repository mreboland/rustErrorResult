fn main() {
    println!("Hello, world!");



    // Error Handling

    // Result

    // Rust doesn't have exceptions. Instead, functions that can fail have a return type that says so:
    fn get_weather(location: LatLng) -> Result<WeatherReport, io::Error>

    // The Result type indicates possible failure. When we call the get_weather() function, it will return either a success result Ok(weather), where weather is a new a new WeatherReport value, or an error result Err(error_value), where error_value is an io::Error explaining what went wrong.

    // Rust requires us to write some kind or error handling whenever we call this function. We can't get at the WeatherReport without doing something to the Result, and we'll get a compiler warning if Result value isn't used. Chapter 10 will expand on Result.



    // Catching Errors

    // The most thorough way of dealing with a Result is the way we showed in chapt 2. Use a match expression.
    match get_weather(hometown) {
        Ok(report) => {
            display_weather(hometown, &report);
        }
        Err(err) => {
            println!("error querying the weather: {}", err);
            schedule_weather_retry();
        }
    }

    // This is Rust's equivalent to try/catch in other languages. It's what you use when you want to handle errors head-on, not pass them on to our caller. match is a bit verbose, so Result<T, E> offers a variety of methods that are useful in particular common cases. Each of these methods has a match expression in its implementation. The methods listed below are the ones most used.
    // 1. result.is_ok() and result.is_err() return a bool telling if result is a success result or an error result.
    // 2. result.ok() returns the success value, if any, as an Option<T>. If result is a success result, this returns Some(success_value); otherwise, it return None, discarding the error value.
    // 3. result.err() returns the error value, if any, as an Option<E>.
    // 4. result.unwrap_or(fallback) returns the success value, if result is a success result. Otherwise, it returns fallback, discarding the error value.

    // A fairly safe prediction for Southern California
    const THE_USUAL: WeatherReport = WeatherReport::Sunny(72);

    // Get a real weather report, if possible.
    // If not, fall back on the usual
    let report = get_weather(los_angeles).unwrap_or(THE_USUAL);
    dispplay_weather(los_angeles, &report);

    // This is a nice alternative to .ok() because the return type is T, not Option<T>. Of course, it only works when there's an appropriate fallback value.
    // 4. result.unwrap_or_else(fallback_fn) is the same, but instead of passing a fallback value directly, we pass a function or closure. This is for cases where it would be wasteful to compute a fallback value if we're not going to use it. The fallback_fn is called only if we have an error result.
    let report = 
        get_weather(hometown)
        .unwrap_or_else(|_err| vague_prediction(hometown));
    // 5. result.unwrap() also return the success value, if result is a success result. However, if result is an error result, this method panics. This method has its uses. More on it later.
    // 6. result.expect(message) is the same as .unwrap(), but lets us provide a message that it prints in case of panic.

    // Lastly, two methods for borrowing references to the value in a Result:
    // 1. result.as_ref() converts a Result<T, E> to a Result<&T, &E>, borrowing a reference to the success or error value in the existing result.
    // 2. result.as_mut() is the same, but borrows a mutable reference. The return type is Result<&mut T, &mut E>.

    // One reason the last two methods are useful is that all the other methods listed, except .is_ok() and .is_err(), consume the result they operate on. That is, they take the self argument by value. Sometimes it's quite handy to access data inside a result without destroying it, and this is what .as_ref() and .as_mut() do for us. For example, suppose we'd like to call result.ok(), but we need result to be left intact. We can write result.as_ref().ok(), which merely borrows result, returning an Option<&T> rather than an Option<T>.



    // Result Type Aliases

    // Sometime we'll see Rust docs that seem to omit the error type of a Result:
    fn remove_file(path: &Path) -> Result<()>

    // This means that a Result type alias is being used.

    // A type alias is a kind of shorthand for type names. Modules often define a Result type alias to avoid having to repeat an error type that's used consistently by almost every function in the module. For example, the standard library's std::io module includes this line of code:
    pub type Result<T> = result::Result<T, Error>;

    // This defines a public type std::io::Result<T>. It's an alias for Result<T, E> but hardcoding std::io::Error as the error type. In practical terms, this means that if we write use std::io; then Rust will understand io::Result<String> as shorthand for Result<String, io::Error>.

    

    // Printing Errors

    // Sometimes the only way to handle an error is by dumping it to the terminal and moving on. We've seen one way already:
    println!("error querying the weather: {}", err);

    // The standard library defines several error types with boring names. std::io::Error, std::fmt::Error, std::str::Utf8Error, and so on. All of them implement a common interface, the std::error::Error trait, which means they share the following features:
    // 1. They're all printable using println!(). Printing an error with the {} format specifier typically displays only a brief error message. Alternatively, we can print with the {:?} format specifier, to get a Debug view of the error. This is less user-friendly, but includes extra technical info.
    // result of `println!("error: {:?}", err);`
    // error: failed to lookup address information: No address associated with hostname

    // result of `println!("error: {:?}", err);`
    // error: Error {repr: Custom(Custom { kind: Other, error: StringError("failed to lookup address information: No address associated with hostname") }) }
    // 2. err.description() returns an error message as a &str
    // 3. err.cause() returns an Option<&Error>: the underlying error, if any, that triggered err.
    // For example, a networking error might cause a banking transaction to fail, which could in turn cause our boat to be repossessed. If err.description() is "boat was repossessed", then err.cause() might return an error about the failed transaction; its .description() might be "failed to transfer $300 to United Yacht Supply", and its .cause() might be an io::Error with details about the specific network outage that caused all the fuss. That third error is the root cause, so its .cause() method would return None.
    // Since the standard library only includes rather low-level features, this is usually None for standard library errors.

    // Printing an error value does not also print out its cause. If we want to be sure to print all the available info, use:
    use std::error::Error;
    use std::io::{Write, stderr};

    /// Dump an error msg to `stderr`.
    ///
    /// If another error happens while building the error msg or
    /// writing to `stderr`, it is ignored.
    fn print_error(mut err: &Error) {
        let _ = writeln!(stderr(), "error: {}", err);
        while let Some(cause) = err.cause() {
            let _ = writeln!(stderr(), "caused by: {}", cause);
            err = cause;
        }
    }

    // The standard library's error types do not include a stack trace, but the error-chain crate makes it easy to define our own custom error type that supports grabbing a stack trace when it's created. It uses the backtrace crate to capture the stack.
}
