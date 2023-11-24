contest=$1

if [ -d "./src/contests/mod_$contest" ]; then
    echo "$name exists."
else
  mkdir "./src/contests/mod_$contest"
  echo "" >> "./src/contests/mod_$contest/mod.rs"
  sed -i "1imod mod_$contest;" ./src/contests/mod.rs
fi

rm ./contest
ln -s $(realpath "./src/contests/mod_$contest") ./contest


