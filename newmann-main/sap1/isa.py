class SAP1ISA:

    OPCODE_TABLE = {
            "LDA": "0000",
            "ADD": "0001",
            "SUB": "0010",
            "OUT": "1110",
            "HLT": "1111"
    }

    ADDR_TABLE = {
            "0H": "0000",
            "1H": "0001",
            "2H": "0010",
            "3H": "0011",
            "4H": "0100",
            "5H": "0101",
            "6H": "0110",
            "7H": "0111",
            "8H": "1000",
            "9H": "1001",
            "AH": "1010",
            "BH": "1011",
            "CH": "1100",
            "DH": "1101",
            "EH": "1110",
            "FH": "1111",
            "XH": "XXXX"
    }
    
    def get_opcode(self, mnemonic: str):
        """ Get the binary opcode equivalence of a mnemonic. """
        return SAP1ISA.OPCODE_TABLE.get(mnemonic)
    
    def get_bin_addr(self, hex_addr: str):
        """ Get the binary address equivalence of a hex address. """
        return SAP1ISA.ADDR_TABLE.get(hex_addr)