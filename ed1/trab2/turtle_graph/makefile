run_command = cargo run --release

testCircle:
	${run_command} -- grammar/circle.lsy circle.ps circle.txt
testKoch:
	${run_command} -- grammar/koch.lsy koch.ps koch.txt
testKoch2:
	BASE_ORDER=4 ${run_command} -- grammar/koch2.lsy koch2.ps koch.txt	
testTree:
	${run_command} -- grammar/tree.lsy tree.ps tree.txt
testTree1:
	${run_command} -- grammar/tree1.lsy tree1.ps tree1.txt
testTree2:
	${run_command} -- grammar/tree2.lsy tree2.ps tree2.txt

clean:
	rm *.ps
	rm *.txt
	cargo clean
	reset