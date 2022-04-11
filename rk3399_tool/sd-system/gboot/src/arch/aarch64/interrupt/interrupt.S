
.global enable_interrupt
enable_interrupt:
	msr	DAIFClr, #0b0010
	RET


.global read_icc_iar1_el1
read_icc_iar1_el1:
	MRS x0, S3_0_C12_C12_0
	RET


.global write_icc_iar1_el1
write_icc_iar1_el1:
	MSR S3_0_C12_C12_1, x0
	MSR S3_0_C12_C11_1, x0
	RET


.global read_spsr_el1
read_spsr_el1:
	MRS x0 ,DAIF
	RET

