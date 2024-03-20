problem=$1

if [ -d "./src/problems/prob_$problem" ]; then
    echo "$problem exists."
else
  mkdir "./src/problems/prob_$problem"
  echo "" >> "./src/problems/prob_$problem/mod.rs"
  sed -i "1imod prob_$problem;" ./src/problems/mod.rs
fi

rm ./problem
ln -s $(realpath "./src/problems/prob_$problem") ./problem
echo "problem synced"


