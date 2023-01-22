import React from 'react'

type UserProp = {
    fieldName: string,
    placeholder: string,
}


const TextAreaForm = (props: UserProp) => {
    return (
        <div className='px-8 pb-8'>
            <div className="text-xl text-primary-content font-semibold">{props.fieldName}</div>
            <div className='pt-2'>
                <textarea rows={5} cols={40} className="textarea textarea-bordered"
                          placeholder={props.placeholder}></textarea>
            </div>
            <button className="btn btn-primary mt-2">Update</button>
        </div>
    )
}

export default TextAreaForm