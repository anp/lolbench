# CPU Shielding

On Linux, this feature uses [cpuset](https://github.com/lpechacek/cpuset) to migrate all non-lolbench processes to other CPUs than the provided range in order to improve reliability of our benchmark measurements. To use this feature you must have cpuset installed and run lolbench under a user account that can run sudo without an interactive prompt. Unless you need to specifically investigate the behavior of the CPU shield, it's recommended to run lolbench without root privileges.
