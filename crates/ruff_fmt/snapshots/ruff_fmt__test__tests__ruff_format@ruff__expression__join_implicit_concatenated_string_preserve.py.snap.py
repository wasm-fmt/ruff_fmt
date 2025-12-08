a = 'different \'quote "are fine"'  # join

# More single quotes
"one single'two 'single' two \"double\""

# More double quotes
'one double"two "double" two \'single\''

# Equal number of single and double quotes
"two \"double\" two 'single'"

# Already invalid Pre Python 312
f"{'Hy "User"'}{"Hy 'User'"}"


# Regression tests for https://github.com/astral-sh/ruff/issues/15514
params = {}
string = f'this is my string with "{params.get("mine")}"'
string = f'"{params.get("mine")} with {"nested single quoted string"}'
string = f"{'''inner ' '''}" f'{"""inner " """}'
string = f"{10 + len('bar')=}{10 + len('bar')=}"
string = f"{10 + len('bar')=}" f'{10 + len("bar")=}'
