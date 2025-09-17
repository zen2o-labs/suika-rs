curl -X POST http://localhost:8080/ingest \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer railway-system-api-key-24680' \
  -d '{
    "isRealTime": true,
    "source": "sensor_A",
    "data": {
      "temperature": 22.5,
      "humidity": 0.45
    },
    "timestamp": "2025-09-17T08:38:04.113506Z",
    "metadata": {
      "location": "room_101",
      "unit": "metric"
    }
  }'
  
curl -X POST http://localhost:8080/ingest \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer railway-system-api-key-24680' \
  -d '{
    "isRealTime": false,
    "source": "weather_station_01",
    "data": {
      "measurements": [
        {"type": "temperature", "value": 18.3, "unit": "celsius"},
        {"type": "wind_speed", "value": 12.5, "unit": "km/h"},
        {"type": "precipitation", "value": 0.0, "unit": "mm"}
      ]
    },
    "timestamp": "2025-09-17T08:30:00.000Z",
    "metadata": {
      "station_id": "WS_001",
      "calibration_date": "2025-09-01",
      "quality_score": 0.98
    }
  }'

curl -X POST http://localhost:8080/ingest \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer railway-system-api-key-24680' \
  -d '{
    "isRealTime": true,
    "source": "train_tracking_system",
    "data": {
      "train_id": "TR_001",
      "current_position": {
        "lat": 28.6139,
        "lon": 77.2090
      },
      "speed": 85.2,
      "next_station": "NEW_DELHI",
      "estimated_arrival": "2025-09-17T09:15:00Z"
    },
    "timestamp": "2025-09-17T08:38:04.500Z",
    "metadata": {
      "route": "RAJDHANI_EXPRESS",
      "operator": "INDIAN_RAILWAYS",
      "tracking_accuracy": "high"
    }
  }'

curl -X POST http://localhost:8080/ingest \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer railway-system-api-key-24680' \
  -d '{
    "isRealTime": false,
    "source": "qaoa_optimizer",
    "data": {
      "problem_type": "TSP",
      "nodes": 10,
      "optimal_cost": 125.7,
      "circuit_depth": 8,
      "success_probability": 0.823,
      "execution_time_ms": 2450
    },
    "timestamp": "2025-09-17T08:35:00.000Z",
    "metadata": {
      "algorithm": "QAOA",
      "p_layers": 4,
      "backend": "qasm_simulator",
      "shots": 8192
    }
  }'

  
  