import init, { format } from "../pkg/ruff_fmt.js";
import { test } from "node:test";
import assert from "node:assert/strict";

await init();

test("it works", () => {
    const input = `x = {  'a':37,'b':42,

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
  return      37+-+a[42-x :  y**3]`;

    const expected = `x = {"a": 37, "b": 42, "c": 927}

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
`;

    const actual = format(input);

    assert.equal(actual, expected);
});
