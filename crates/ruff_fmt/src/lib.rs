use ruff_python_formatter::format_module;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(input: &str) -> Result<String, String> {
    format_module(input, Default::default())
        .map(|result| result.as_code().to_string())
        .map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::format;

    #[test]
    fn it_works() {
        let input = r###"x = {  'a':37,'b':42,

'c':927}

y = 'hello ''world'
z = 'hello '+'world'
a = 'hello {}'.format('world')
class foo  (     object  ):
  def f    (self   ):
    return       37*-+2
  def g(self, x,y=42):
      return y
def f  (   a ) :
  return      37+-+a[42-x :  y**3]"###;

        let expected = r###"x = {"a": 37, "b": 42, "c": 927}

y = "hello " "world"
z = "hello " + "world"
a = "hello {}".format("world")


class foo(object):
    def f(self):
        return 37 * -+2

    def g(self, x, y=42):
        return y


def f(a):
    return 37 + -+a[42 - x : y**3]
"###;

        let actual = format(input).unwrap();

        assert_eq!(actual, expected);
    }
}
