aws ec2 run-instances \
--image-id ami-08d0b940d663d5ea0 \
--count 1 \
--instance-type r6g.xlarge \
--key-name mac \
--security-group-ids sg-094c40f8d41b6d337 \
--enclave-options 'Enabled=true'

# m5.xlarge x86 4 vCPU 16 GiB 0.264USD
# r6g.xlarge ARM 4 vCPU 32 GiB 0.268USD
# ami-0178f90ef28e37dfc x86
# ami-08d0b940d663d5ea0 ARM
