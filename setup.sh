setup_day () {
	[ -d "day_$1" ] && echo "Directory day_$1 exists." && return
	cp -r day_0 day_$1
	sed -i '' "s/day_0/day_$1/g" day_$1/Cargo.toml
	sed -i '' "s/day 0/day $1/g" day_$1/src/main.rs	
}

for i in $(seq 1 9); do setup_day 0$i; done
for i in $(seq 10 25); do setup_day $i; done