import { Connection } from './index'

/*
	This is a quick test of the library.
	The objetive is to check if the bindings are working
*/

async function listMachines () {
  const conn = Connection.open('qemu:///system')
  if (conn.isAlive()) {
    console.log('Connection is alive')
  } else {
    console.log('Connection is not alive')
    return
  }
  console.log(await conn.getNodeInfo())
  console.log(Number(await conn.getFreeMemory()) / (1024 * 1024 * 1024)) // In GB

  const machines: Array<string> = await conn.listDefinedDomains() // Assuming 0 is a flag for all machines
  console.log('Available Machines:', machines)
}

listMachines().catch(console.error)
