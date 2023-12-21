#[ -d "day_$1" ] && echo "Directory day_$1 exists." && exit

#cp -r day_0 day_$1
#sed -i '' "s/day_0/day_$1/g" day_$1/Cargo.toml
#sed -i '' "s/day 0/day $1/g" day_$1/src/main.rs
#code day_$1

sed -i '' '1s/^/use shared::Solution;\'$'\n/g' day_$1/src/solutions.rs
sed -i '' s/"pub fn pt_1(str_input: \&str) {"/"pub fn pt_1(str_input: \&str) -> Solution {"/g day_$1/src/solutions.rs
# sed -i '' s/"pub fn pt_2(str_input: \&str) {"/"pub fn pt_2(str_input: \&str) -> Solution {"/g day_$1/src/solutions.rs