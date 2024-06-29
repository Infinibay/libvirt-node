import test from 'ava'
import { Libvirt } from '..'

test('connect to Libvirt', async t => {
  const libvirt = new Libvirt();
  await t.notThrowsAsync(async () => {
    return libvirt.connect("test-connection-string");
  });
});

test('define a domain from XML', async t => {
  const libvirt = new Libvirt();
  await libvirt.connect("test-connection-string"); // Ensure connection
  await t.notThrowsAsync(async () => {
    return libvirt.defineXML("<domain type='test'><name>testvm</name></domain>");
  });
});

test('power on a machine', async t => {
  const libvirt = new Libvirt();
  await libvirt.connect("test-connection-string"); // Ensure connection
  await t.notThrowsAsync(async () => {
    return libvirt.powerOn("testvm");
  });
});

test('power off a machine', async t => {
  const libvirt = new Libvirt();
  await libvirt.connect("test-connection-string"); // Ensure connection
  await t.notThrowsAsync(async () => {
    return libvirt.powerOff("testvm", true); // ACPI enabled
  });
});
