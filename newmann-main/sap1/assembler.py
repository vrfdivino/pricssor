from isa import SAP1ISA


class SAP1Assembler:

    def __init__(self, source_code: list[str]):
        self.isa = SAP1ISA()
        self.machlang = list(map(self.__assembly_to_machlang, source_code))

    def __assembly_to_machlang(self, line_of_code: str):
        """ Transform a line of assembly code to machine language. """
        sanitized = line_of_code.strip()
        mnemonic = sanitized.split(" ")[0]
        try:
            hex_addr = sanitized.split(" ")[1]
        except:
            hex_addr = "XH"
        opcode = self.isa.get_opcode(mnemonic.strip().upper())
        bin_addr = self.isa.get_bin_addr(hex_addr.strip().upper())
        if not opcode or not bin_addr:
            raise Exception("Invalid assembly code!")
        return opcode + bin_addr
    
if __name__ == "__main__":
    source_code = [
        "LDA 9H",
        "ADD AH",
        "ADD BH",
        "SUB CH",
        "OUT",
        "HLT"
    ]
    assembler = SAP1Assembler(source_code)
    assert assembler.machlang[0] == "00001001"
    assert assembler.machlang[1] == "00011010"
    assert assembler.machlang[2] == "00011011"
    assert assembler.machlang[3] == "00101100"
    assert assembler.machlang[4] == "1110XXXX"
    assert assembler.machlang[5] == "1111XXXX"