mkdir -p docker_context
cd docker_context
mkdir -p executables

#build the instructor binaries
git clone https://github.com/replicatedu/replicatedu_instructor.git
cd replicatedu_instructor
cargo build --release 
cp target/release/replicatedu_instructor ../executables
cd ..

#build the student binaries
git clone https://github.com/replicatedu/replicatedu_student.git
cd replicatedu_student
cargo build --release
cp target/release/replicatedu_student ../executables
cd ..

#build the test runner
git clone https://github.com/replicatedu/test_runner.git
cd test_runner
cargo build --release
cp target/release/test_runner ../executables
cd ..