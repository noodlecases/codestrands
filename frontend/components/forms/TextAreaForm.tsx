import React, {useState} from 'react'
import {useRouter} from "next/router";

type UserProp = {
    fieldName: string,
    placeholder: string,
    callFunction: Function | null,
}


const TextAreaForm = (props: UserProp) => {
    const router = useRouter()

    const[value, setValue] = useState("")

    const handleInput = event => {
        setValue(event.target.value)
    }

    const callInput = () => {
        if (props.callFunction != null) {
            props.callFunction(value)
            router.reload()
        }
    }

    return (
        <div className='px-8 pb-8'>
            <div className="text-xl text-primary-content font-semibold">{props.fieldName}</div>
            <div className='pt-2'>
                <textarea rows={5} cols={40} className="textarea textarea-bordered"
                          onChange={handleInput} placeholder={props.placeholder}></textarea>
            </div>
            <button onClick={callInput} className="btn btn-primary mt-2">Update</button>
        </div>
    )
}

export default TextAreaForm