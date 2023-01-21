import React from 'react'
import Navbar from '../components/Navbar'

const chat = () => {
  return (
    <div className = "flex justify-center">
        <div>
          <Navbar />
        </div>
        <div className = "h-screen w-[50%] border-base-content border-x-2">
            <div className='flex'>
                <ul className="menu bg-base-300 w-56">
                    <li><a>Item 1</a></li>
                    <li><a>Item 2</a></li>
                    <li><a>Item 3</a></li>
                </ul>
                <div className='h-screen w-full border-base-content border-l-2'>
                    <div className='flex justify-center h-12 m-4'>
                        Name
                    </div>
                    <div className="chat chat-start">
                        <div className="chat-bubble">It's over Anakin, <br/>I have the high ground.</div>
                    </div>
                    <div className="chat chat-end">
                        <div className="chat-bubble">You underestimate my power!</div>
                    </div>

                    <div className='absolute bottom-0'>
                        <div className="form-control">
                            <div className="input-group">
                                <input type="text" placeholder="Send a message" className="input input-bordered" />
                                <button className="btn btn-square">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                                        <path strokeLinecap="round" strokeLinejoin="round" d="M6 12L3.269 3.126A59.768 59.768 0 0121.485 12 59.77 59.77 0 013.27 20.876L5.999 12zm0 0h7.5" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div>
          test
        </div>
    </div>
  )
}

export default chat