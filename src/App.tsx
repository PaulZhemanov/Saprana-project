import React from 'react';
import styled from "@emotion/styled";
import {Text} from "./components/Text";
import {Header} from "./components/Header";
import Window from "./components/Window";
import SizedBox from "./components/SizedBox";
import background0 from "./assets/images/background0.png"
import background1 from "./assets/images/background1.png"
import {Column} from './components/Flex';
import {Tag} from "./components/Tag";

const Root = styled.div`
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
  max-width: 100%;
  min-width: 1440px;
  min-height: 100vh;
  padding-bottom: 0;
  justify-content: flex-start;
  flex-shrink: 0;

  border: 10px solid #574EF1;
  background: rgba(87, 78, 241, 0.20);
`;

const Body = styled.div`
  display: flex;
  align-items: stretch;
  justify-content: stretch;
  width: 100%;
  max-height: 1024px;
  height: 100%;
  padding: 18px;
  flex: 1;
  box-sizing: border-box;

`


function App() {
    return (
        <Root>
            <Header>
                <Text style={{flex: 1, minWidth: "fit-content"}}>📍NYC / Amsterdam / London</Text>
                <Text style={{flex: 1}}>Jess 美 Chang</Text>
                <Text style={{flex: 1}}>@jchangsta</Text>
            </Header>
            <Body>
                <Column style={{flex: 1}} mainAxisSize="stretch" crossAxisSize="max">
                    <Window title="About me" background={background0}>
                        <Text>3 playful speculative objects, happy accidents, geometric patterns, grid systems, learning
                            about cultures & kind people. I enjoy thinking about new ways to make, but equally if not
                            more
                            enjoy applying ideas to the real world.</Text>
                        <SizedBox height={18}/>
                        <Window style={{height: 270}} background={background1} title="This is me!"> </Window>
                    </Window>
                    <SizedBox height={22}/>
                    <Window title="Words that describe me" bodyStyle={{flexDirection: 'row'}}>
                        <Tag>Relational</Tag>
                        <Tag>Analytical</Tag>
                        <Tag>Executor</Tag>
                    </Window>
                </Column>
                <SizedBox width={22}/>
                <Column style={{width: 187, height: 904}} justifyContent="center" alignItems="center">
                    <Window title="Talk to me about" style={{width: 904, transform: 'rotate(-90deg)'}}
                            bodyStyle={{flexDirection: 'row'}}>
                        <Tag>Visual design</Tag>
                        <Tag>Design systems</Tag>
                        <Tag>Figma</Tag>
                    </Window>
                </Column>
                <SizedBox width={22}/>
                <Column style={{flex: 1}}>
                    <Window title="Where’s home?">
                        <Text>where i have community</Text>
                    </Window>
                    <SizedBox height={22}/>
                    <Window title="Secret power">
                        <Text>I can beat you in a game of knock out :)</Text>
                    </Window>
                    <SizedBox height={22}/>
                    <Window title="I’m learning...">
                        <Tag>To tell better stories</Tag>
                        <SizedBox height={16}/>
                        <Tag>Hardware prototyping</Tag>
                        <SizedBox height={16}/>
                        <Tag>Writing better frontend</Tag>
                        <SizedBox height={16}/>
                        <Tag>How to write clearly</Tag>
                    </Window>

                </Column>

            </Body>
        </Root>
    );
}

export default App;
