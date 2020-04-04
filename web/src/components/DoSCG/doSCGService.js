export const doSCGService = async () => {
  const res = await fetch('/api/doscg')
 
  return res.json()
}

export default doSCGService