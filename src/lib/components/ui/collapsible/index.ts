import { Collapsible as CollapsiblePrimitive } from 'bits-ui';
import Content from './collapsible-content.svelte';

const Root = CollapsiblePrimitive.Root;
const Trigger = CollapsiblePrimitive.Trigger;

const Collapsible = {
  Root,
  Content,
  Trigger
};

export default Collapsible;
