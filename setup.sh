[ -d "day_$1" ] && echo "Directory day_$1 exists." && exit

cp -r day_01/ day_$1/
sed -i '' "s/day_01/day_$1/g" day_$1/Cargo.toml
sed -i '' "s/day 1/day $1/g" day_$1/src/main.rs
#code day_$1
#cd day_$1
