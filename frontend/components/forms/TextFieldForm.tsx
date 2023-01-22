import React from 'react'

type UserProp = {
    fieldName: string,
    placeholder: string,
}


const TextFieldForm = (props: UserProp) => {
    return (
        <div className='px-8 pb-8'>
            <div className="text-xl text-primary-content font-semibold">{props.fieldName}</div>
            <div className='flex pt-2'>
                <input type="text" placeholder={props.placeholder} className="input input-bordered w-full max-w-xs mr-2" />
                <button className="btn btn-primary">Update</button>
            </div>
        </div>
    )
}

export default TextFieldForm