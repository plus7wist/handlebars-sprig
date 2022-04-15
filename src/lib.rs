use handlebars::Handlebars;

pub mod date;
pub mod math;
pub mod strings;

pub fn addhelpers(x: &mut Handlebars) {
    strings::addhelpers(x);
    math::addhelpers(x);
    date::addhelpers(x);
}
