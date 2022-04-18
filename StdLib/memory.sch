#macro MODIFY_TARGET(_VAR_)
__MEM_TYPE(_VAR_), ` `, __MEM_LOCATION(_VAR_)
#endmacro

#macro MODIFY_FROM(_VAR_, _TARGET_, _PATH_)
write `data modify `, MODIFY_TARGET(_VAR_), ` `, _VAR_, ` set from `, _TARGET_, ` `, _PATH_
#endmacro

#macro DATA_SET_VALUE(_VAR_, _VALUE_)
write `data modify `, MODIFY_TARGET(_VAR_), ` `, _VAR_, ` set value `, _VALUE_
#endmacro

#macro DATA_SET_FROM(_VAR_, _REF_)
MODIFY_FROM(_VAR_, MODIFY_TARGET(_REF_), _REF_)
#endmacro

#macro DATA_SET(_VAR_, _TO_)
#if __TYPE(_TO_) == "REF"
DATA_SET_FROM(_VAR_, _TO_)
#else
DATA_SET_VALUE(_VAR_, _TO_)
#endif
#endmacro

