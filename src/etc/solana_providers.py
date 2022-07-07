import lldb


def encode_b58(num):
    alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
    b58 = ""
    while num != 0:
        b58 += alphabet[num % 58]
        num //= 58
    return b58[::-1]

def PubkeySummaryProvider(valobj, internal_dict):
    err = lldb.SBError()
    if valobj.TypeIsPointerType():
        program_id_type = valobj.GetType().GetPointeeType()
        value = valobj.GetPointeeData()
    else:
        value = valobj.GetData()
    pubkey = [value.GetUnsignedInt8(err, i) for i in range(32)]
    pubkey = ''.join("{0:0{1}x}".format(i, 2) for i in pubkey)
    pubkey = encode_b58(int(pubkey, 16))

    if valobj.TypeIsPointerType():
        return "-> Pubkey = {}".format(pubkey)
    return "{}".format(pubkey)


def AccountInfoSummaryProvider(valobj, internal_dict):
    err = lldb.SBError()
    # key
    key_valobj = valobj.GetChildAtIndex(0)

    # is_signer
    signer_valobj = valobj.GetChildAtIndex(1)

    # is_writeable
    writeable_valobj = valobj.GetChildAtIndex(2)

    # lamports
    lamport_valobj = valobj.GetChildAtIndex(3)
    lamport_valobj = lamport_valobj.GetChildAtIndex(0)
    lamport_valobj = lamport_valobj.GetChildAtIndex(0)
    lamport_valobj = lamport_valobj.GetChildAtIndex(0)
    lamports_data = lamport_valobj.GetData();
    lamports = lamports_data.GetUnsignedInt64(err, 0)

    # data
    data_valobj = valobj.GetChildAtIndex(4)
    data_valobj = data_valobj.GetChildAtIndex(0)
    data_valobj = data_valobj.GetChildAtIndex(0)
    data_valobj = data_valobj.__str__().replace("value", "data")
    data_valobj = data_valobj.__str__().replace("  [", "        [")
    data_valobj = data_valobj.__str__().replace("}", "    }")

    # owner
    owner_valobj = valobj.GetChildAtIndex(5)

    # executable
    executable_valobj = valobj.GetChildAtIndex(6)

    # rent_epoch
    rent_epoch_valobj = valobj.GetChildAtIndex(7)
    rent_epoch_data = rent_epoch_valobj.GetData();
    rent_epoch = rent_epoch_data.GetUnsignedInt64(err, 0)

    ret_string = ""
    if valobj.TypeIsPointerType():
        ret_string = "-> "

    return ret_string + """{{
  {}
  {}
  {}
  (u64) lamports = {}
  {}
  {}
  {}
  (u64) rent_epoch = {}
}}
            """.format(key_valobj, signer_valobj, writeable_valobj, lamports, owner_valobj, executable_valobj, data_valobj, rent_epoch)
