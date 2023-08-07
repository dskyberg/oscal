# Rust support for OSCAL

## codegen
Generates Rust code from the full schema in the [schema](./schema) folder

## oscal_lib
The main oscal lib.  The code for this lib was bootstrapped with [codegen](./codegen)

## POAM
Basic support for reading and generating POAM files.  The first pass provides support for reading a POAM from a .csv file and generating an OSCAL equivilant.