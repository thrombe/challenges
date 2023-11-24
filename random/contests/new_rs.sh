name=$1

if [ -f "./contest/$name.rs" ]; then
    echo "$name exists."
else 
    cp ./src/lib/template_v3.rs "./contest/$name.rs"
    touch "./contest/$name.txt"
    sed -i "1imod $name;" ./contest/mod.rs
fi

rm p.rs
ln -s $(realpath ./contest/$name.rs) ./p.rs
rm input.txt
ln -s $(realpath ./contest/$name.txt) ./input.txt

