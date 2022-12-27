install:
	cargo install --path .

echo-many-things:
	echo something
	echo "something else"
	echo "this" &&
		echo "this as well"

echo-one-thing:
	echo "one thing" --and-this-flag


lsesses:
	ls