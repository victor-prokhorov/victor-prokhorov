import React, { ReactNode } from 'react';
import './App.css';
import Highcharts from 'highcharts';
import HighchartsReact from 'highcharts-react-official';
import { ReactComponent as TrendArrowUp } from './assets/trend-arrow-up.svg';
import { ReactComponent as TrendArrowDown } from './assets/trend-arrow-down.svg';
// import Card from './components/Card';

const options = {
  chart: {
    type: 'spline',
  },
  title: {
    text: 'My chart',
  },
  series: [
    {
      data: [1, 2, 1, 4, 3, 6],
    },
    {
      data: [2, 4, 2, 5, 4, 7],
    },
  ],
};
const transactionsPerWeekDay: Highcharts.Options = {
  chart: {
    type: 'column',
  },
  exporting: {
    enabled: false,
  },
  credits: {
    enabled: false,
  },
  series: [
    {
      showInLegend: false,
      data: [39000, 10000, 25000, 39500, 26000, 50000, 0],
      type: 'column',
      name: 'Transactions',
      color: '#006d44',
    },
  ],
  subtitle: {
    text: '',
  },
  title: {
    text: '',
  },
  xAxis: {
    categories: ['M', 'T', 'W', 'T', 'F', 'S', 'S'],
    labels: {
      style: {
        fontSize: '10px',
      },

    },

  },

  yAxis: {

    title: {

      text: '',

    },

    labels: {

      format: '{value}',

      style: {

        fontSize: '10px',

      },

    },

    tickInterval: 10000,

  },

};
interface CardContainerProps {
  children?: ReactNode;
}

function CardContainer({ children }: CardContainerProps) {
  return <div className="card-container">{children}</div>;
}

interface CardProps {
  title: string;
  value: string;
  previousValue: string;
  trend: 'up' | 'down';
}

function Card({ title, value, trend, previousValue }: CardProps) {
  return (
    <div className="card">
      <p className="card__title">{title}</p>
      <p className="card__value">{value}</p>
      <div className="card__trend">
        <div className="card__trend__icon">
          {trend === 'up' ? <TrendArrowUp /> : <TrendArrowDown />}
        </div>
        <p>vs previous year</p>
      </div>
      <div>
        <HighchartsReact highcharts={Highcharts} options={options} />
      </div>
    </div>
  );
}

function App() {
  return (
    <div className="App">
      <CardContainer>
        <Card title="Title" value="1.234" trend="up" previousValue="0.7%" />
        <Card title="Title" value="$123K" trend="down" previousValue="0.7%" />
      </CardContainer>
    </div>
  );
}

export default App;
