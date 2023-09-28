if ! command -v python3 &> /dev/null
then
    echo "python3 could not be found"
    exit 1
fi

if ! command -v cmake &> /dev/null
then
    echo "cmake could not be found"
    exit 1
fi

if ! command -v git &> /dev/null
then
    echo "git could not be found"
    exit 1
fi

python -m venv rdf4cpp_venv # create venv
source rdf4cpp_venv/bin/activate # activate venv

pip install "conan<2" # install conan

# setup conan
conan user || true
conan profile new --detect rdf4cpp_0_0_17 || true
conan profile update settings.compiler.libcxx=libstdc++11 rdf4cpp_0_0_17 || true
conan remote add -f dice-group https://conan.dice-research.org/artifactory/api/conan/tentris || true
