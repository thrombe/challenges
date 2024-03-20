# name=$1
name="p"

if [ -f "./problem/$name.rs" ]; then
    echo "$name.rs exists."
else 
    cp ./src/lib/template_v3.rs "./problem/$name.rs"
    touch "./problem/$name.txt"
    sed -i "1imod $name;" ./problem/mod.rs
fi

rm p.rs
ln -s $(realpath ./problem/$name.rs) ./p.rs
rm input.txt
ln -s $(realpath ./problem/$name.txt) ./input.txt
echo "rust synced"

