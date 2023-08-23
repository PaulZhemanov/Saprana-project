import styled from "@emotion/styled";

export const Header = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-sizing: border-box;
  padding: 0 24px;
  width: 100%;
  min-width: fit-content;
  height: 80px;
  flex-shrink: 0;

  border-bottom: 10px solid #574EF1;
  background: #FFF;

  & > :nth-of-type(2) {
    text-align: center;
  }

  & > :nth-of-type(3) {
    text-align: right;
  }
`;
