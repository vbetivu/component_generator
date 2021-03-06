import {shallow} from "enzyme";
import {expect} from "chai";

import {{component}} from "./{{component}}";

describe('{{component}}', () => {
  const wrapper = shallow(<{{component}} />);

  it('should render properly', () => {
    expect(wrapper.is(".{{classname}}")).to.be.true;
  });
});