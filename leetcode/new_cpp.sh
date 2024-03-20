# name=$1
name="p"

if [ -f "./problem/$name.cpp" ]; then
    echo "$name.cpp exists."
else 
    cp ./src/lib/template_v0.cpp "./problem/$name.cpp"
    touch "./problem/$name.txt"
fi

rm p.cpp
ln -s $(realpath ./src/lib/solve.cpp) ./p.cpp
rm input.txt
ln -s $(realpath ./problem/$name.txt) ./input.txt
echo "cpp synced"

