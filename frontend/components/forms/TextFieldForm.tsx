import React, {useState} from 'react'

type UserProp = {
    fieldName: string,
    placeholder: string,
    callFunction: Function | null,
}

const TextFieldForm = (props: UserProp) => {
    const[value, setValue] = useState("")

    const handleInput = event => {
        setValue(event.target.value)
    }

    const callInput = () => {
        if (props.callFunction != null)
            props.callFunction(value)
    }

    return (
        <div className='px-8 pb-8'>
            <div className="text-xl text-primary-content font-semibold">{props.fieldName}</div>
            <div className='flex pt-2'>
                <input type="text" onChange={handleInput} placeholder={props.placeholder} className="input input-bordered w-full max-w-xs mr-2" />
                <button onClick={callInput} className="btn btn-primary">Update</button>
            </div>
        </div>
    )
}

export default TextFieldForm