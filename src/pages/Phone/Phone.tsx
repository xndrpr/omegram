export default function Phone() {
  return (
    <div className='h-screen w-screen flex flex-col gap-2 justify-center items-center bg-black text-white'>
        <input placeholder='Phone' className='bg-inherit rounded-lg p-2 outline-none border-2 border-gray-800 text-gray-500 placeholder-slate-800'/>
        <button className='p-2 bg-gray-800 rounded-lg hover:opacity-80 transition-all duration-200 ms border-2 border-gray-800'>Continue</button>
    </div>
  )
}
