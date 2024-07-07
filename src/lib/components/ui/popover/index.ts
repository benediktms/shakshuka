import { Popover as PopoverPrimitive } from 'bits-ui';
import Content from './popover-content.svelte';
const Root = PopoverPrimitive.Root;
const Trigger = PopoverPrimitive.Trigger;
const Close = PopoverPrimitive.Close;

const Popover = {
  Root,
  Content,
  Trigger,
  Close
};

export default Popover;
