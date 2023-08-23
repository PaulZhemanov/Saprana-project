import styled from "@emotion/styled";
import React, {PropsWithChildren} from "react";
import {Row} from "./Flex";
import {Header} from "./Header";
import crossIcon from "../assets/icons/CloseIcon.svg"
import hideIcon from "../assets/icons/HideIcon.svg"
import expandIcon from "../assets/icons/ExpandIcon.svg"
import {Text} from "./Text";

const Icon = styled.div`
  width: 32px;
  height: 32px;
  margin-right: 12px;
`

const CrossIcon = styled(Icon)`
  background: url(${crossIcon}) center no-repeat;
`
const HideIcon = styled(Icon)`
  background: url(${hideIcon}) center no-repeat;
`
const ExpandIcon = styled(Icon)`
  background: url(${expandIcon}) center no-repeat;
`

interface IProps extends PropsWithChildren {
    title?: string
    background?: string
    align?: "row" | 'column'
    style?: React.CSSProperties
    bodyStyle?: React.CSSProperties
}

const Root = styled.div`
  display: flex;
  flex-direction: column;
  width: 100%;
  border: 10px solid #574EF1;
  box-sizing: border-box;
`;

const Body = styled.div<{ background?: string }>`
  display: flex;
  flex-direction: column;
  padding: 16px;
  height: 100%;
  box-sizing: border-box;

  ${({background}) => background ? `background: url(${background}), repeat` : ""};
`


const Window: React.FC<IProps> = ({title = "", children, background, bodyStyle, ...rest}) => {
    return <Root {...rest}>
        <Header>
            <Row>
                <CrossIcon/>
                <HideIcon/>
                <ExpandIcon/>
            </Row>
            <Text style={{whiteSpace: 'nowrap'}}>{title}</Text>
        </Header>
        <Body background={background} style={bodyStyle}>
            {children}
        </Body>
    </Root>;
}

export default Window;
